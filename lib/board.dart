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
    return Scaffold(
      body: Center(
        child: Column(
          children: [
            for (int row = 0; row < _board.length; row++)
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  for (int col = 0; col < _board[row].length; col++)
                    Padding(
                      padding: const EdgeInsets.all(8.0),
                      child: ElevatedButton(
                        onPressed: () {
                          _incrementCell(row, col);
                        },
                        child: Text(
                          _board[row][col].toString(),
                        ),
                      ),
                    ),
                ],
              ),
          ],
        ),
      ),
    );
  }
}
