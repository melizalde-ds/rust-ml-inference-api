# 🚀 Rust ML Inference API

A high-performance, memory-safe API for serving machine learning model inferences, built with Rust, Axum, and ONNX
Runtime. Designed for fast cloud deployment and scalable AI systems.

---

## ✨ Features

- 🦀 Built with async Rust (Tokio + Axum)
- 🧠 ONNX model inference serving (MNIST classifier)
- 🐳 Docker-ready for easy deployment
- ☁️ Cloud-compatible (Containers)
- 🛡️ Defensive input validation and error handling

---

## 📚 Tech Stack

- Rust
- Axum (Web framework)
- Tokio (Async runtime)
- ONNX Runtime
- Docker

---

## 🚀 Running Locally

1. Clone the repo:

```bash
git clone https://github.com/melizalde-ds/rust-ml-inference-api.git
cd rust-ml-inference-api
```

Server will start at:
http://localhost:3000

## 📬 API Endpoints

POST /predict

    Request: JSON body with flattened 28x28 grayscale image (784 float32 values)

    Response: Predicted probabilities for digits 0–9

Example request:

```json
{
  "values": [
    0.0,
    0.1,
    0.2,
    0.0,
    ...,
    (784
    floats
    total)
  ]
}
```

Example curl:

```bash
curl -X POST http://localhost:3000/predict -H "Content-Type: application/json" -d '{"values": [0.0, 0.1, 0.2, 0.0, ..., 0.0]}'
```

GET /healthz

    Response: OK

## 🐳 Running with Docker

Build the image:

    docker build -t rust-ml-inference-api .

Run the container:

    docker run -p 3000:3000 rust-ml-inference-api

Access your API at:
http://localhost:3000

## 📦 Model

The ONNX model (mnist-8.onnx) is stored under /onnx_models/ for easy access.

Original
source: [ONNX Model Zoo - MNIST 8](https://github.com/onnx/models/blob/main/validated/vision/classification/mnist/model/mnist-8.onnx)

## 🚀 Future Improvements

- Structured logging with tracing
- Support dynamic model outputs
- Optimize concurrency for batch inference
- Model hot-reloading
- Improve ONNX runtime threading configuration

## 📄 License

MIT License

## 🤝 Connect with me!

Always happy to collaborate on Rust, Cloud, or AI projects!
Feel free to reach out on LinkedIn!