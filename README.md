# URL Shortener made in Rust

A URL shortener service built with Rust backend, React frontend, and PostgreSQL database. It is for study purposes and is not intended for production use.

## Architecture

- **Backend**: Rust with Axum framework
- **Frontend**: React with TypeScript and Vite
- **Database**: PostgreSQL
- **Reverse Proxy**: Nginx
- **Containerization**: Docker and Docker Compose

## How to Run

### Prerequisites

- Docker
- Docker Compose

### Running the Application

1. Clone the repository:
```bash
git clone <repository-url>
cd url-shortener-rust
```

2. Start all services with Docker Compose:
```bash
docker-compose up --build
```

3. Access the application:
   - **Frontend (URL Shortener Interface)**: http://localhost
   - **Backend API**: http://localhost/api/

### Services

The application runs with the following services:

- **Nginx** (Port 80): Reverse proxy that routes requests
  - `/` → Frontend (React app)
  - `/api/*` → Backend API
  - `/{shortCode}` → Backend (for URL redirects)
  - Static assets → Frontend

- **Frontend** (Internal Port 5173): React application with Vite dev server
- **Backend** (Internal Port 3000): Rust API server
- **Database** (Port 5432): PostgreSQL database

### Usage

1. Open http://localhost in your browser
2. Enter a long URL in the input field
3. Click "Shorten" to generate a shortened URL
4. Use the shortened URL to redirect to the original URL

### API Endpoints

- `POST /api/shorten` - Create a shortened URL
- `GET /{shortCode}` - Redirect to original URL

### Development

To stop the services:
```bash
docker-compose down
```

To rebuild and restart:
```bash
docker-compose up --build
```

To view logs:
```bash
docker-compose logs -f [service-name]
```
