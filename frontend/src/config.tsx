// Base URL for backend API
// In production, you might want to use environment variables
// like import.meta.env.VITE_BACKEND_URL
export const BACKEND_URL =
  import.meta.env.BACKEND_URL || "http://localhost:3000";

// API endpoints
export const API_ENDPOINTS = {
  shorten: `${BACKEND_URL}/api/shorten`,
};
