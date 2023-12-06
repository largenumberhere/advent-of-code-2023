namespace day4_part1;


internal static class Program
{
    public static int Main(string[] args)
    {
        // get input
        string[]? lines = Lines.GetFileLines(args);
        if (lines == null) {
            Console.WriteLine("Usage: program.exe ./input-file.txt");
            return -1;
        }
        
        // convert to scratchboard object
        List<ScratchBoardRow> rows = new List<ScratchBoardRow>();
        foreach (var line in lines) {
            Lines.LineChopper lineChopper = new(line);
            Lines.LineParser lineParser = new(lineChopper);
        
            ScratchBoardRow row = new ScratchBoardRow(lineParser);
            
            rows.Add(row);
        }
        Scratchboard scratchboard = new Scratchboard(rows);
        
        // Find the winners
        Scratchboard.RowWinners[] winners = scratchboard.GetWinners();
        
        // Add up the winners' values
        int score = 0;
        foreach (var row in winners)
        {
            score += row.GetRowScore();
        }

        Console.WriteLine(@"Elf scratchcard value: {0}", score);
        
        return 0;
    }
}