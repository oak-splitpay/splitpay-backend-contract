/**
 * Payment entity representing individual payments within a split
 */
export class Payment {
  id: string;
  splitId: string;
  senderAddress: string;
  amount: number;
  asset: string;
  status: 'pending' | 'confirmed' | 'failed';
  transactionHash?: string;
  ledger?: number;
  createdAt: Date;
  confirmedAt?: Date;
  errorMessage?: string;
}
