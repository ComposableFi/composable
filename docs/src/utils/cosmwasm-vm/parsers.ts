import { RawContractEvent, ContractEvent } from './types';

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
