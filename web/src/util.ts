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

export const getData = async (input: RequestInfo) => {
	return fetch(input, { method: 'get' }).then(handleNetworkResponse);
};
export const postData = async (input: RequestInfo, data: any) => {
	return fetch(input, {
		method: 'post',
		headers: {
			'Content-Type': 'application/json;charset=utf-8',
		},
		body: JSON.stringify(data),
	}).then(handleNetworkResponse);
};

// Convert from an Object to an Array, and run a closure on each entry.
export const objectMapToArray = (
	objectMap: Record<string, Record<string, unknown>>,
	entryClosure: (entry: Record<string, unknown>) => void,
) => {
	const outputArray = [];
	for (const entryId in objectMap) {
		const entry = Object.assign({}, objectMap[entryId]);

		entryClosure(entry);

		outputArray.push(entry);
	}
	return outputArray;
};

export const sanitiseObjectMapToArray = (objectMap: Record<string, Record<string, unknown>>) => {
	return objectMapToArray(objectMap, (entry) => {
		if (!entry.dataValue) entry.dataValue = '';
		else {
			// @ts-ignore
			entry.dataValue = entry.dataValue.trim();
		}
	});
};

export const timeout = (ms: number) => {
	return new Promise((resolve) => setTimeout(resolve, ms));
};

// Verifies that the local credentials are still valid. If they aren't, it redirects to the login page.
// There's no security issue here, as all API requests also require a valid token anyway.
// All it does is prevent unsightly errors for the user.
export const redirectIfNotLoggedIn = () => {
	const loggedInCheckUrl = '/api/loggedIn';
	const loginLocation = '/login';
	return getData(loggedInCheckUrl).then((loggedIn) => {
		if (!loggedIn.ok || !loggedIn.value) window.location.replace(loginLocation);
	});
};

// Utility function to display a value as empty if it's null
export const emptyIfNull = (value: string) => {
	return value != null ? value : '';
};
