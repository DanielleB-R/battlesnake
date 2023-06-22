ECR_URL ?= 002682819933.dkr.ecr.us-west-2.amazonaws.com/core/battlesnake
# Version is done like this because the docker images are tagged by the commit hash, so if you just pass it v0.1.2 it won't find an image
# IWe have to search the repo for what commit a particular tag refers to.
VERSION := $(if $(VERSION),$(shell git rev-parse --short $(VERSION) || echo $(VERSION)),$(shell git rev-parse --short HEAD))
COMMON_MANIFESTS := deployment service ingress

.PHONY: kubectl-setup
kubectl-setup:
	aws eks update-kubeconfig --region us-west-2 --name venture --alias integration
	kubectl config use-context integration

.PHONY: deploy
deploy: ## Deploy Kubernetes resources
	$(call check_variable, AWS_REGION, You must specify the region to use (eg. 'us-east-1'))
	$(call check_variable, ENVIRONMENT, You must specify the environment to use (eg. 'sandbox'))
	@$(MAKE) -f Makefile $(foreach manifest,$(COMMON_MANIFESTS),deploy-manifest-$(manifest))
	kubectl rollout restart --namespace battlesnake deployment/$(SNAKE_NAME)
	kubectl rollout status --namespace battlesnake deployment/$(SNAKE_NAME)

.PHONY: deploy-manifest-%
deploy-manifest-%: ## Deploy a Kubernetes manifest
	@ECR_URL=$(ECR_URL) \
	VERSION=$(VERSION) \
		iidy render \
			--region us-west-2 \
			--environment integration \
			infrastructure/kube/$*.yaml \
		| kubectl apply -f -

.PHONY: cleanup
cleanup: ## Remove all Kubernetes resources
	$(call check_variable, AWS_REGION, You must specify the region to use (eg. 'us-east-1'))
	$(call check_variable, ENVIRONMENT, You must specify the environment to use (eg. 'sandbox'))
	ECR_URL=$(ECR_URL) \
	VERSION=$(VERSION) \
		iidy render \
			--region us-west-2 \
			--environment integration \
			infrastructure/kube/ \
		| kubectl delete -f -

ecr-login: # Login to ECR
	aws ecr get-login-password --region us-west-2 | docker login --password-stdin --username AWS 002682819933.dkr.ecr.us-west-2.amazonaws.com

.PHONY: debug
debug: ## Show status of all Kubernetes resources
	kubectl describe pods --namespace battlesnake

.PHONY: release
release: ecr-login ## Push Docker container to ECR
	docker push $(ECR_URL):$(SNAKE_NAME)-$(VERSION)

.PHONY: run
run: ## Run application in Docker container
	docker run --rm -v $(SNAKE_NAME):latest

.PHONY: build
build: ## Build Docker container
	docker build -f snakes/$(SNAKE_NAME)/Dockerfile -t $(ECR_URL):$(SNAKE_NAME)-$(VERSION) -t $(SNAKE_NAME):latest ./snakes/$(SNAKE_NAME)

# No-op targets to fulfill standard makefile contract

.PHONY: help
help: ## Display this message
	@grep --no-filename -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| sort | awk 'BEGIN {FS = ":.*?## "} \
		{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'