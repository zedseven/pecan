// Imports
import { get, writable } from 'svelte/store';
import { handleNetworkResponse, Ok } from './util';
import type { ResponseError, Result } from './util';

// Selected Location
const storedLocation = JSON.parse(localStorage.getItem('location') || JSON.stringify(null));
export const selectedLocation = writable(storedLocation);
selectedLocation.subscribe((value) => {
	localStorage.setItem('location', JSON.stringify(value));
});

// Print Settings
const defaultPrintSettings = {
	horizontalLabelCount: 2,
	verticalLabelCount: 3,
	slot: 1,
	labelMargin: 5 /* Millimetres */,
	fontSize: 13 /* Points */,
	borderMarkers: true,
};
const storedPrintSettings = JSON.parse(
	localStorage.getItem('printSettings') || JSON.stringify(defaultPrintSettings),
);
export const printSettings = writable(storedPrintSettings);
printSettings.subscribe((value) => {
	localStorage.setItem('printSettings', JSON.stringify(value));
});

// Definitions (column definitions, locations, etc.) from the database
const definitions = writable(null);
let fetchDefinitionsPromise: Promise<Result<any, ResponseError>> | null = null;
export const fetchDefinitions = async () => {
	// If the definitions have already been retrieved, return them immediately
	const value = get(definitions);
	if (value) {
		// console.log('Already fetched - returning immediately.');
		return Ok(value);
	}
	// console.log('Not yet fetched - returning promise.');

	// Otherwise, fetch them from the server
	const definitionsUrl = '/api/devices/definitions';
	if (!fetchDefinitionsPromise)
		fetchDefinitionsPromise = fetch(definitionsUrl, { method: 'get' })
			.then(handleNetworkResponse)
			.then(async (responseData) => {
				// console.log('Definitions have been fetched.');
				if (responseData.ok) {
					definitions.set(responseData.value);
				}
				// console.log(responseData);
				return responseData;
			});
	return fetchDefinitionsPromise;
};
