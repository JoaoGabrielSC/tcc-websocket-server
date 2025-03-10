
run:
	@echo "Running Application"
	@cargo run

websocket:
	@echo "Running websocket"
	@websocat websocat ws://127.0.0.1:3030/ws
