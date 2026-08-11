package main

type CheckoutPort interface {
	ProcessOrder(order OrderPayload) (OrderResult, error)
	CancelOrder(orderID string) (bool, error)
}
