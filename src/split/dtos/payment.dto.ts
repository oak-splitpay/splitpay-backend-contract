/**
 * DTO for recording a payment
 */
export class RecordPaymentDto {
  splitId: string;
  senderAddress: string;
  transactionHash: string;
  ledger: number;
}
