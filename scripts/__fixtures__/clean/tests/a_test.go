func TestOk(t *testing.T) {
	got := add(2, 3)
	if got != 5 {
		t.Fatalf("want 5, got %d", got)
	}
}
