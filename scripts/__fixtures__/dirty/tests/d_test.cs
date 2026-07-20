public class Dirty
{
    [Fact(Skip = "flaky")]
    public void X()
    {
        Thread.Sleep(2000);
    }
}
