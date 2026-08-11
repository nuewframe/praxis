export interface CheckoutPort {
  processOrder(order: OrderPayload): Promise<OrderResult>;
  cancelOrder(orderId: string): Promise<boolean>;
}
