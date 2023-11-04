import 'package:flutter/material.dart';

class Board extends StatefulWidget {
  const Board({Key? key}) : super(key: key);

  @override
  State<Board> createState() => _BoardState();
}

class _BoardState extends State<Board> {
  // clickable button in board increments cell value
  List<List<int>> _board = [
    [0, 0, 0],
    [0, 0, 0],
    [0, 0, 0],
  ];

  void _incrementCell(int row, int col) {
    setState(() {
      _board[row][col]++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return AspectRatio(
      aspectRatio: 1.0,
      child: GridView.builder(
        gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
          crossAxisCount: _board.length,
        ),
        itemBuilder: _buildGridItems,
        itemCount: _board.length * _board[0].length,
      ),
    );
  }

  Widget _buildGridItems(BuildContext context, int index) {
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
}
