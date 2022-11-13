using System;
using System.Runtime.InteropServices;
using System.Text;

class FFI
{
  public delegate string TauriCallback(IntPtr msg);

  [DllImport("src-tauri/target/debug/libtauri_dotnet.dylib", EntryPoint = "run_tauri")]
  public static extern uint RunTauri(TauriCallback cb);

  [DllImport("src-tauri/target/debug/libtauri_dotnet.dylib", EntryPoint = "message_free")]
  internal static extern void MessageFree(IntPtr song);
}

internal class Message : SafeHandle
{
  public Message() : base(IntPtr.Zero, true) { }
  public Message(IntPtr ptr) : base(ptr, true) { }

  public override bool IsInvalid
  {
    get { return this.handle == IntPtr.Zero; }
  }

  public string AsString()
  {
    int len = 0;
    while (Marshal.ReadByte(handle, len) != 0) { ++len; }
    byte[] buffer = new byte[len];
    Marshal.Copy(handle, buffer, 0, buffer.Length);
    return Encoding.UTF8.GetString(buffer);
  }

  protected override bool ReleaseHandle()
  {
    if (!this.IsInvalid)
    {
      FFI.MessageFree(handle);
    }

    return true;
  }
}