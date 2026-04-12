import { Injectable, NotFoundException } from '@nestjs/common';
import { v4 as uuidv4 } from 'uuid';
import { InMemoryDatabaseService } from '../database/in-memory.service';
import { CreateSplitDto } from './dtos/create-split.dto';
import { RecordPaymentDto } from './dtos/payment.dto';
import { Split } from './entities/split.entity';
import { Payment } from './entities/payment.entity';

@Injectable()
export class SplitService {
  constructor(private readonly db: InMemoryDatabaseService) {}

  createSession(data: CreateSplitDto) {
    const splitId = uuidv4();
    const amountPerPerson = Number((data.totalAmount / data.participants).toFixed(7));

    const split: Split = {
      id: splitId,
      title: data.title,
      totalAmount: data.totalAmount,
      asset: data.asset || 'USDC',
      participants: data.participants,
      amountPerPerson,
      receiverAddress: data.receiverAddress,
      status: 'active',
      payments: [],
      createdAt: new Date(),
      updatedAt: new Date(),
      expiresAt: new Date(Date.now() + 7 * 24 * 60 * 60 * 1000), // 7 days
    };

    this.db.create('splits', splitId, split);

    return {
      message: 'Split session created',
      splitId,
      shareableLink: `https://splitpay.app/pay/${splitId}`,
      split,
    };
  }

  getSession(id: string): Split {
    const split = this.db.findById<Split>('splits', id);
    if (!split) throw new NotFoundException('Split session not found');
    return split;
  }

  recordPayment(data: RecordPaymentDto): Payment {
    const split = this.getSession(data.splitId);

    const paymentId = uuidv4();
    const payment: Payment = {
      id: paymentId,
      splitId: data.splitId,
      senderAddress: data.senderAddress,
      amount: split.amountPerPerson,
      asset: split.asset,
      status: 'confirmed',
      transactionHash: data.transactionHash,
      ledger: data.ledger,
      createdAt: new Date(),
      confirmedAt: new Date(),
    };

    this.db.create('payments', paymentId, payment);

    // Update split's payments
    split.payments.push(paymentId);
    this.db.update('splits', split.id, { payments: split.payments });

    return payment;
  }

  getPayments(splitId: string): Payment[] {
    this.getSession(splitId); // Validate split exists
    const allPayments = this.db.findAll<Payment>('payments');
    return allPayments.filter(p => p.splitId === splitId);
  }

  getSplitStatus(id: string) {
    const split = this.getSession(id);
    const payments = this.getPayments(id);

    const totalReceived = payments
      .filter(p => p.status === 'confirmed')
      .reduce((sum, p) => sum + p.amount, 0);

    const progress = (payments.length / split.participants) * 100;

    return {
      splitId: id,
      status: split.status,
      totalAmount: split.totalAmount,
      amountPerPerson: split.amountPerPerson,
      totalReceived,
      paymentsReceived: payments.filter(p => p.status === 'confirmed').length,
      totalParticipants: split.participants,
      progress: Math.round(progress),
      payments: payments.map(p => ({
        id: p.id,
        senderAddress: p.senderAddress,
        amount: p.amount,
        status: p.status,
        confirmedAt: p.confirmedAt,
      })),
    };
  }
}