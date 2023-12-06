using System.Text;

namespace day4_part1;

record class Scratchboard
{
    public ScratchBoardRow[] Rows { get; }

    public Scratchboard(IEnumerable<ScratchBoardRow> rows)
    {
        this.Rows = rows.ToArray();
    }

    public RowWinners[] GetWinners()
    {
        var rowsWinners = new List<RowWinners>();
        for (int rowNumber = 0; rowNumber < Rows.Length; rowNumber++)
        {
            var row = Rows[rowNumber];
            
            var winners = new List<int>();
            for (int i = 0; i < row.NumbersGiven.Length; i++)
            {
                var number = row.NumbersGiven[i];
                
                foreach (var winning in row.WinningNumbers)
                {
                    if (number == winning)
                    {
                        var newWinner = (winning);
                        winners.Add(newWinner);
                    }
                }
            }

            var rowWinners = new RowWinners(rowNumber, winners.ToArray(), winners.Count);
            rowsWinners.Add(rowWinners);
        }

        return rowsWinners.ToArray();
    }

    public record RowWinners(int RowNumber, int[] WinningNumbers, int WinCount)
    {
        public int GetRowScore()
        {
            int score = 0;
            if (WinCount > 0)
            {
                // first winner adds 1 point
                score = 1;
                
                // each one after doubles the points
                for(int i = 0; i < WinCount - 1; i++)
                {
                    score *= 2;
                }           
            }

            return score;
        }
    }

    public override string ToString()
    {
        StringBuilder builder = new StringBuilder();

        builder.Append("Scratchboard {");
        
        for (int i = 0; i < this.Rows.Length; i++)
        {
            builder.Append('\n');
            builder.Append('\t');
            builder.AppendFormat("Row {0}: ", i);
            builder.Append('\'');
            builder.AppendFormat("Card {0, 2}:  ",this.Rows[i].CardNumber);
            
            
            foreach (int number in this.Rows[i].NumbersGiven)
            {
                builder.Append(' ');
                builder.AppendFormat("{0, 2}", number);
            }

            builder.Append(" | ");

            foreach (int number in this.Rows[i].WinningNumbers)
            {
                builder.Append(' ');
                builder.AppendFormat("{0, 2}", number);
            }
            
            builder.Append('\'');
        }
        
        builder.Append('\n');
        builder.Append('}');

        return builder.ToString();
    }
}