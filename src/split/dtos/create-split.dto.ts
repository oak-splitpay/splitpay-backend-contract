/**
 * DTO for creating a new split
 */
export class CreateSplitDto {
  title: string;
  totalAmount: number;
  asset: string; // e.g., 'USDC', 'XLM'
  participants: number;
  receiverAddress: string;
}
