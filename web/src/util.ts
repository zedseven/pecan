// Rust-style `Result` type.
//
// https://www.huy.rocks/everyday/02-14-2022-typescript-implement-rust-style-result
export type Result<T, E = undefined> = { ok: true; value: T } | { ok: false; error: E | undefined };

// A function that creates a `Result` value indicating success.
export const Ok = <T>(data: T): Result<T, never> => {
	return { ok: true, value: data };
};

// A function that creates a `Result` value indicating failure.
export const Err = <E>(error?: E): Result<never, E> => {
	return { ok: false, error };
};

// An HTTP response error, including the status code, status description, and error message.
export type ResponseError = {
	status: number;
	statusText: string;
	message: string;
};

// Takes a `fetch` response and maps it to a `Result` with the parsed JSON body on success, or an
// error with the pertinent information on failure.
export const handleNetworkResponse = async (response: Response) => {
	if (!response.ok) {
		return Err({
			status: response.status,
			statusText: response.statusText,
			message: await response.text(),
		});
	}
	return Ok(await response.json());
};

export const postData = async (input: RequestInfo, data: any) => {
	return fetch(input, {
		method: 'post',
		headers: {
			'Content-Type': 'application/json;charset=utf-8',
		},
		body: JSON.stringify(data),
	});
};
