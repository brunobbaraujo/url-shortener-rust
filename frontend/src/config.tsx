// Base URL for backend API
// In production, you might want to use environment variables
// like import.meta.env.VITE_BACKEND_URL
export const BACKEND_URL = "http://localhost:8000/api";

// API endpoints
export const API_ENDPOINTS = {
  shorten: `${BACKEND_URL}/shorten`,
};
