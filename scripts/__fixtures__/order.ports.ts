export interface OrderPort {
  createOrder(payload: any): Promise<boolean>;
  cancelOrder(id: string): Promise<boolean>;
}
