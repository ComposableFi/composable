import { RawContractEvent, ContractEvent } from './types';
import deburr from 'lodash/deburr';

export function hexToString(hex: string): string {
	let str = '';
	for (let n = 0; n < hex.length; n += 2) {
		const int = parseInt(hex.substr(n, 2), 16);
		//	to prevent \u0001 & \u0007 appearing in front
		if (int !== 0 && int !== 7) str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
	}
	console.log(hex, str);
	return str;
}

/* eslint-disable no-param-reassign */
export const unhexifyKeys = (data: Record<string, any>): Record<string, any> => {
	Object.keys(data).forEach((key: string) => {
		data[hexToString(key)] = data[key];
		delete data[key];
	});
	return data;
};
export function jsonReplacer(key: string, value: any) {
	if (value instanceof Map) {
		return {
			dataType: 'Map',
			value: Array.from(value.entries()), // or with spread: value: [...value]
		};
	}
	return value;
}

export function jsonReviver(key: string, value: any) {
	if (typeof value === 'object' && value !== null) {
		if (value.dataType === 'Map') {
			return new Map(value.value);
		}
	}
	return value;
}

export function getRefinedEvents(events: RawContractEvent[]): ContractEvent[] {
	const refinedEvents: ContractEvent[] = events.map(event => {
		const ret: ContractEvent = {
			type: event.type,
			attributes: {},
		};
		event.attributes.forEach(attr => {
			ret.attributes[attr.key] = attr.value;
		});
		return ret;
	});
	return refinedEvents;
}
