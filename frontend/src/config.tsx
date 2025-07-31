// Base URL for backend API
// Uses BACKEND_URL environment variable from docker-compose
export const BACKEND_URL =
  import.meta.env.BACKEND_URL || "http://localhost:3000";

// API endpoints
export const API_ENDPOINTS = {
  shorten: `${BACKEND_URL}/api/shorten`,
};
