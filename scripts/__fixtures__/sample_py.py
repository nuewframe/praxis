class CheckoutPort:
    def process_order(self, order: dict) -> dict:
        pass

    def cancel_order(self, order_id: str) -> bool:
        pass
