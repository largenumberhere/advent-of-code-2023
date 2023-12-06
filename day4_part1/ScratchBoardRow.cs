namespace day4_part1;

record class ScratchBoardRow
{
    public int CardNumber { get; }
    public int[] WinningNumbers { get; }
    public int[] NumbersGiven { get; }

    public ScratchBoardRow(Lines.LineParser lineParser)
    {
        this.CardNumber = lineParser.CardNumber;
        this.NumbersGiven = lineParser.NumbersLeft;
        this.WinningNumbers = lineParser.NumbersRight;
    }

}