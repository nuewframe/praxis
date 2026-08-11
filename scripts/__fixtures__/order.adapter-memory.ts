export class OrderMemoryAdapter {
  createOrder(payload: any): Promise<boolean> {
    return Promise.resolve(true);
  }
  cancelOrder(id: string): Promise<boolean> {
    return Promise.resolve(true);
  }
}
