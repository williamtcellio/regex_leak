using System;
using System.Diagnostics;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;
using Newtonsoft.Json;

namespace DotnetTest.ConsoleApp
{
    public class CreateAppFirewallResponse
    {
        public string Error { get; set; }
        [JsonProperty(PropertyName = "appfirewall_ptr")]
        public IntPtr? AppFirewallPtr { get; set; }
    }

    public class AppFirewallInspectResponse
    {
        public string Error { get; set; }
        public AppFirewallDetection[] Detections { get; set; }

        public AppFirewallInspectResponse()
        {
            Detections = new AppFirewallDetection[] { };
        }
    }

    public class AppFirewallDetection
    {
        [JsonProperty(PropertyName = "dp")]
        public string DetectionPoint { get; set; }

        [JsonProperty(PropertyName = "pattern_id")]
        public string PatternId { get; set; }
    }
    internal class TcellAgentNative
    {
        [DllImport(@"D:\Git\libtcellagent\target\release\tcellagent.dll", EntryPoint = "appfirewall_init", CharSet = CharSet.Auto)]
        public static extern int AppFirewallInit(byte[] pattern, long patternSize, byte[] policy, long policyLength, byte[] buffer, long bufferMaxSize);

        [DllImport(@"D:\Git\libtcellagent\target\release\tcellagent.dll", ThrowOnUnmappableChar = true, BestFitMapping = false, EntryPoint = "appfirewall_inspect")]
        public static extern int AppFirewallInspect(IntPtr appFirewall, byte[] message, long messageSize, byte[] buffer, long bufferMaxSize);

        [DllImport(@"D:\Git\libtcellagent\target\release\tcellagent.dll", ThrowOnUnmappableChar = true, BestFitMapping = false, EntryPoint = "appfirewall_free")]
        public static extern int AppFirewallFree(IntPtr appFirewall);
    }

    class Program
    {
        public static long NumberOfTimes = 10000;
        
        static void Main(string[] args)
        {
            var pattern = ReadFile("pattern.json");
            var policy = ReadFile("policy.json");
            var request = ReadFile("request.json");

            var buffer = new byte[512];
            var result = TcellAgentNative.AppFirewallInit(pattern, pattern.LongLength, policy, policy.LongLength, buffer, buffer.LongLength);
            var agent = JsonConvert.DeserializeObject<CreateAppFirewallResponse>(Encoding.UTF8.GetString(buffer, 0, result));
            
            for (var i = 0U; i < NumberOfTimes; ++i)
            {
                var t = new Thread(() =>
                {
                    Execute(agent, request);
                });
                t.Start();
                t.Join();
            }
            TcellAgentNative.AppFirewallFree(agent.AppFirewallPtr.Value);
            Console.ReadKey();
        }
        
        private static void Execute(CreateAppFirewallResponse agent, byte[] request)
        {
            var buffer = new byte[512];
            var result = TcellAgentNative.AppFirewallInspect(agent.AppFirewallPtr ?? IntPtr.Zero, request, request.LongLength, buffer, buffer.LongLength);
            var apa = JsonConvert.DeserializeObject<AppFirewallInspectResponse>(Encoding.UTF8.GetString(buffer, 0, result));
            if (!string.IsNullOrWhiteSpace(apa.Error))
                throw new InvalidOperationException(apa.Error);
        }
        private static byte[] ReadFile(string file)
        {
            var text = System.IO.File.ReadAllText(file, Encoding.UTF8);
            return Encoding.UTF8.GetBytes(text);
        }
    }
}
