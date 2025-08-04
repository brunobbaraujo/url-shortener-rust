// Base URL for backend API
// Uses current origin (nginx proxy) to access backend
export const BACKEND_URL = import.meta.env.VITE_BACKEND_URL || window.location.origin;

// API endpoints
export const API_ENDPOINTS = {
  shorten: `${BACKEND_URL}/api/shorten`,
};
