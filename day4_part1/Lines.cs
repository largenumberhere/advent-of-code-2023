using System.Text;

namespace day4_part1;

/// <summary>
///  String utilities related to the problem
/// </summary>
static class Lines
{
    public class LineParser
    {
        public int CardNumber { get; }
        public int[] NumbersLeft { get; }
        public int[] NumbersRight { get; }

        public LineParser(LineChopper lineParts)
        {
            string rawCardNumber = lineParts.CardNumber;
            string rawLeftNumbers = lineParts.NumbersLeft;
            string rawRightNumbers = lineParts.NumbersRight;

            try
            {
                CardNumber = int.Parse(rawCardNumber);
            }
            catch (Exception e)
            {
                Console.WriteLine(@"Failed to parse card number '{0}'. left: '{1}'. right: '{2}'", rawCardNumber, rawLeftNumbers, rawRightNumbers);
                throw;
            }

            List<int> leftNumbers = new List<int>();
            //string[] leftNumbersSplit = rawLeftNumbers.Split(" ");
            string[] leftNumbersSplit = SplitWhitespace(rawLeftNumbers);
            foreach (var t in leftNumbersSplit)
            {
                int number = int.Parse(t);
                leftNumbers.Add(number);
            }

            NumbersLeft = leftNumbers.ToArray();

            List<int> rightNumbers = new List<int>();
            string[] rightNumbersSplit = SplitWhitespace(rawRightNumbers);
            foreach (var t in rightNumbersSplit)
            {
                string trimmed = t.Trim();
                int number = int.Parse(trimmed);
                rightNumbers.Add(number);
            }

            NumbersRight = rightNumbers.ToArray();
        }
        
        /// Each time there is at least one whitespace, create a new string from the characters before it. All strings returned will have at least Length 1
        string[] SplitWhitespace(string values)
        {
            StringBuilder pending = new StringBuilder();
            List<string> outputs = new List<string>();
            foreach (char c in values)
            {
                if (char.IsWhiteSpace(c))
                {
                    if (pending.Length > 0)
                    {
                        outputs.Add(pending.ToString());
                        pending = new StringBuilder();
                    }
                    
                    continue;
                }

                pending.Append(c);
            }

            if (pending.Length > 0)
            {
                outputs.Add(pending.ToString());
            }

            return outputs.ToArray();
        }
    }

    public class LineChopper
    {
        public string CardNumber { get; }
        public string NumbersLeft { get; }
        public string NumbersRight { get; }

        public LineChopper(string line)
        {
            string[] allLeftAndRight = line.Split("|");
            string[] cardAndLeft = allLeftAndRight[0].Split(":");

            string left = cardAndLeft[1];
            string cardPhraseAndNumber = cardAndLeft[0];
            string right = allLeftAndRight[1];

            string[] card = cardPhraseAndNumber.Split(" ");
            string cardNumber = card.Last(); // If there is more that whitespace, it will be added to the middle of this array.
            
            CardNumber = cardNumber;
            NumbersLeft = left;
            NumbersRight = right;
        }
    }

    public static string[]? GetFileLines(string[] programArgs)
    {
        if (programArgs.Length != 1)
        {
            return null;
        }

        string[]? lines = null;
        try
        {
            lines = File.ReadAllLines(programArgs[0]);
        }

        catch (System.IO.FileNotFoundException)
        {
            Console.WriteLine("File not found. Exiting...");
        }

        return lines;
    }
}