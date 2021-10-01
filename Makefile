export CONTAINER_REGISTRY := ""

.PHONY: docker-run-test
docker-run: docker build nauttilus/rust-test 
	
