﻿class Program
{
  [STAThread]
  static void Main(string[] args)
  {
    FFI.RunTauri((ptr) =>
    {
      var msg = new Message(ptr);
      Console.WriteLine($"dotnet: from rust: {msg.AsString()}");
      msg.Dispose();

      return "hi!";
    });
    Console.ReadKey();
  }
}