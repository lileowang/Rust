using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;
using static System.Console;

namespace ConsoleApp1
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SampleStruct
    {
        public Int16 field_one;
        public Int32 field_two;
    }

    class Program
    {
        [DllImport("rust_engine.dll", EntryPoint = "add_numbers")]
        public static extern uint Add_numbers(uint a, uint b);

        [DllImport("rust_engine.dll", EntryPoint = "get_sample_struct")]
        public static extern SampleStruct Get_sample_struct();

        static void Main(string[] args)
        {
            WriteLine("========================================");
            WriteLine("call rust add_numbers() function:");
            var sum = Add_numbers(1, 2);
            WriteLine(sum);

            WriteLine("========================================");
            WriteLine("call rust get_sample_struct() function:");
            var sample_struct = Get_sample_struct();
            WriteLine(sample_struct.field_one);
            WriteLine(sample_struct.field_two);

            WriteLine("press Enter to quit...");
            ReadLine();
        }
    }
}
