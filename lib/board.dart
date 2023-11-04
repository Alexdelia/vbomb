import 'package:flutter/material.dart';

typedef Tile = int;
typedef BoardT = List<List<Tile>>;

typedef Clue = int;

class Board extends StatefulWidget {
  const Board({Key? key}) : super(key: key);

  @override
  State<Board> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  final BoardT _board = [
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
    [0, 0, 0, 0],
  ];

  final List<Clue> verticalClue = [1, 2, 3, 4];
  final List<Clue> horizontalClue = [1, 2, 3, 4];

  void _incrementCell(int row, int col) {
    setState(() {
      _board[row][col]++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return AspectRatio(
      aspectRatio: 1.0,
      child: Column(
        children: [
          Expanded(
            flex: _board.length,
            child: Row(
              children: [
                Expanded(flex: _board.length, child: _innerBoard()),
                Expanded(child: _sideClueList(Axis.vertical, verticalClue)),
              ],
            ),
          ),
          Expanded(
              child: Row(
            children: [
              Expanded(
                  flex: _board.length,
                  child: _sideClueList(Axis.horizontal, horizontalClue)),
              Expanded(child: _clueItem(42))
            ],
          )),
        ],
      ),
    );
  }

  Widget _innerBoard() {
    return GridView.builder(
      gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: _board.length,
      ),
      itemBuilder: _buildGridItem,
      itemCount: _board.length * _board[0].length,
    );
  }

  Widget _buildGridItem(BuildContext context, int index) {
    final row = index ~/ _board.length;
    final col = index % _board.length;

    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: ElevatedButton.icon(
        onPressed: () {
          _incrementCell(row, col);
        },
        icon: const Icon(Icons.add),
        label: Text(_board[row][col].toString()),
      ),
    );
  }

  Widget _sideClueList(Axis direction, List<Clue> clue) {
    // return ListView.builder(
    //   scrollDirection: direction,
    //   itemCount: clue.length,
    //   itemBuilder: (context, index) {
    //     return _clueItem(clue[index]);
    //   },
    // );
    return GridView.builder(
      gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
        crossAxisCount: 1,
      ),
      scrollDirection: direction,
      itemBuilder: (context, index) {
        return _clueItem(clue[index]);
      },
      itemCount: clue.length,
    );
  }

  Widget _clueItem(Clue clue) {
    return Padding(
      padding: const EdgeInsets.all(8.0),
      child: ElevatedButton.icon(
        onPressed: () {
          print(clue);
        },
        icon: const Icon(Icons.ac_unit),
        label: Text(clue.toString()),
      ),
    );
  }
}
