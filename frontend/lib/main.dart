import 'package:flutter/material.dart';

void main() {
  runApp(const TokamakApp());
}

class TokamakApp extends StatelessWidget {
  const TokamakApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'TOKAMAK64',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: Colors.deepPurple,
          brightness: Brightness.dark,
        ),
        useMaterial3: true,
      ),
      home: const TokamakHome(),
    );
  }
}

class TokamakHome extends StatefulWidget {
  const TokamakHome({Key? key}) : super(key: key);

  @override
  State<TokamakHome> createState() => _TokamakHomeState();
}

class _TokamakHomeState extends State<TokamakHome> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('TOKAMAK64'),
        centerTitle: true,
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            const Text(
              'TOKAMAK64 Game Client',
              style: TextStyle(fontSize: 24, fontWeight: FontWeight.bold),
            ),
            const SizedBox(height: 24),
            const Text(
              'Rust FFI bindings ready',
              style: TextStyle(fontSize: 16, color: Colors.grey),
            ),
            const SizedBox(height: 48),
            ElevatedButton(
              onPressed: () {
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(content: Text('Game not yet implemented')),
                );
              },
              child: const Text('Start Game'),
            ),
          ],
        ),
      ),
    );
  }
}
