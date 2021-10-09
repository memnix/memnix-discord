start: build_release start_service
restart: build_release restart_service

build_release: 
	cargo build --release 
run_debug:
	cargo run
run_release:
	./target/release/memnix
start_service:
	sudo systemctl start memnixbot.service 
stop_service:
	sudo systemctl stop memnixbot.service 
status_service:
	sudo systemctl status memnixbot.service
restart_service:
	sudo systemctl restart memnixbot.service 

