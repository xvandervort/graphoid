# Toronto Maple Leafs Statistics Analyzer

A comprehensive statistical analysis of the Toronto Maple Leafs' performance from 2014-2024, demonstrating Graphoid's Class-Like Graphs (CLG) feature.

## Run

```bash
graphoid samples/06-projects/leafs_stats/main.gr
```

## Features Demonstrated

- **Class-Like Graphs (CLG)**: `LeafsAnalyzer` as a stateful object with methods
- **Computed Properties**: `season_count`, `total_wins`, `first_season`, etc.
- **Private Methods**: Helper functions like `_format_pct()`, `_repeat()`
- **DataSet Integration**: Statistical analysis using the stdlib `dataset` module
- **JSON Data Loading**: Real curated data from NHL sources
- **Rich Reporting**: Comprehensive analysis output

## Data Source

The dataset in `data/maple_leafs_2014_2024.json` was curated from:
- [NHL Records](https://records.nhl.com)
- [Hockey-Reference](https://www.hockey-reference.com)
- [HockeyFights](https://www.hockeyfights.com)

## Statistics Analyzed

- **Team Records**: Wins, losses, OT losses, points per season
- **Home vs Away**: Performance differential at home vs road
- **Goals**: Scoring and defense trends
- **Fighting**: Fights per season (showing league-wide decline)
- **Playoff Performance**: Made/missed, round exits, Game 7 record
- **Scoring Leaders**: Top point-getters by season (Matthews vs Marner)

## The Story in the Data

This 10-year window captures the Leafs' rebuild:

1. **2014-16**: The tank years (68-69 points)
2. **2016-17**: Rookie explosion (Matthews, Marner, Nylander)
3. **2017-24**: Contender status but playoff heartbreak
4. **2021-22**: Matthews MVP season (60 goals)
5. **2023-24**: Matthews' 69-goal franchise record

The data reveals the cruel pattern: 7 playoff appearances, 5 first-round exits, 3 Game 7 losses to Boston alone.

## Project Structure

```
leafs_stats/
├── main.gr              # Entry point
├── leafs_analyzer.gr    # CLG module with analysis methods
├── data/
│   └── maple_leafs_2014_2024.json  # Curated dataset
└── README.md
```
