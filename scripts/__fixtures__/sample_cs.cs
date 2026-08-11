public interface ICheckoutPort {
    Task<OrderResult> ProcessOrderAsync(OrderPayload order);
    Task<bool> CancelOrderAsync(string orderId);
}
