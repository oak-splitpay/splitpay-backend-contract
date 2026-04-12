/**
 * Split entity representing a bill split session
 */
export class Split {
  id: string;
  title: string;
  totalAmount: number;
  asset: string; // Stellar asset code (e.g., 'USDC')
  participants: number;
  amountPerPerson: number;
  receiverAddress: string;
  status: 'pending' | 'active' | 'completed' | 'cancelled';
  payments: string[]; // Payment IDs
  createdAt: Date;
  updatedAt: Date;
  expiresAt: Date;
}
