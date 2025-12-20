/**
 * Common error handling utilities
 */

/** Unified error type for API errors */
export interface ApiError {
  message: string;
  code?: number | string;
  severity?: string;
  recoverable?: boolean;
  details?: unknown;
}

/** Type guard to check if an error is an ApiError */
export function isApiError(error: unknown): error is ApiError {
  return (
    typeof error === 'object' &&
    error !== null &&
    'message' in error &&
    typeof (error as ApiError).message === 'string'
  );
}

/** NeuraDock backend CommandError payload (Tauri command error) */
export interface CommandErrorPayload {
  code: number;
  message: string;
  severity: string;
  recoverable: boolean;
}

export function isCommandErrorPayload(error: unknown): error is CommandErrorPayload {
  return (
    typeof error === 'object' &&
    error !== null &&
    'code' in error &&
    typeof (error as CommandErrorPayload).code === 'number' &&
    'message' in error &&
    typeof (error as CommandErrorPayload).message === 'string' &&
    'severity' in error &&
    typeof (error as CommandErrorPayload).severity === 'string' &&
    'recoverable' in error &&
    typeof (error as CommandErrorPayload).recoverable === 'boolean'
  );
}

function unwrapTauriError(error: unknown): unknown {
  if (error && typeof error === 'object') {
    const maybeWithError = error as { error?: unknown };
    if (maybeWithError.error) return maybeWithError.error;
  }
  return error;
}

/** Extract error message from various error types */
export function extractErrorMessage(
  error: unknown,
  fallback = 'Unknown error'
): string {
  const unwrapped = unwrapTauriError(error);
  if (isCommandErrorPayload(unwrapped)) {
    return unwrapped.message;
  }
  if (unwrapped instanceof Error) {
    return unwrapped.message;
  }
  if (isApiError(unwrapped)) {
    return unwrapped.message;
  }
  if (typeof unwrapped === 'string') {
    return unwrapped;
  }
  return fallback;
}

/** Convert unknown error to ApiError */
export function toApiError(error: unknown): ApiError {
  const unwrapped = unwrapTauriError(error);
  if (isCommandErrorPayload(unwrapped)) {
    return unwrapped;
  }
  if (isApiError(unwrapped)) {
    return unwrapped;
  }
  if (unwrapped instanceof Error) {
    return {
      message: unwrapped.message,
      details: unwrapped,
    };
  }
  if (typeof unwrapped === 'string') {
    return {
      message: unwrapped,
    };
  }
  return {
    message: 'Unknown error',
    details: unwrapped,
  };
}
