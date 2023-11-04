import 'package:flutter/material.dart';
import 'package:vbomb/board.dart';
import 'package:vbomb/rest.dart';

class PlayPage extends StatelessWidget {
  const PlayPage({super.key});

  @override
  Widget build(BuildContext context) {
    return const _PositionManager(
      children: [
        Board(),
        Expanded(child: Rest()),
      ],
    );
  }
}

class _PositionManager extends StatelessWidget {
  const _PositionManager({required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return OrientationBuilder(
      builder: (context, orientation) {
        return orientation == Orientation.portrait
            ? Column(
                children: children,
              )
            : Row(
                children: children,
              );
      },
    );
  }
}
