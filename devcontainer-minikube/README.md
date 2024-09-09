# devcontainer-helm-minikube

Demo project that shows how to work with minikube & helm inside devcontainer.

This demo use [devcontainer template](https://github.com/devcontainers/templates/tree/main/src/kubernetes-helm-minikube) to setup all required things.

Devcontainer features might not work with all images as usually they require `apt` installed in guest container to install additional software.

## quick start

```
minikube start
minikube image build . -t spring-petclinic:latest
cd helm
helm install spring-petclinic spring-petclinic
minikube service --all
```

## notes

Minikube needs to be started manually via `minikube start`

In order to be able to use locally built images with minikube you need to setup correct docker environment. It can be done by executing following command: `eval $(minikube -p minikube docker-env)`. Then you need to built local images that you want to use for deployment into minikube cluster. Additionally, `imagePullPolicy` should be set to `Never` or `IfNotPresent`, otherwise kubernetes will try to pull image anyway. Alternatively, you can use following command to move locally built image to minikube docker env: `minikube image load <image-name>:<image-version>`. Note that in this case spring-petclinic needs to be built first with following command: `docker build . -t spring-petclinic:latest`.
Alternatively, you can use minikube docker environment to build image directly by using following `minikube image build . -t spring-petclinic:latest`

To expose ClusterIP services for local development you might run: `minikube service <name>|--all`. Make sure that ports are then forwarded to the host which should be done automatically.