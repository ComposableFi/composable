import { createHash } from 'sha256-uint8array';
import { codeStore } from './methods';

export const sha256 = (input: Uint8Array) => {
	return createHash().update(input).digest('hex');
};

export const loadRemoteContract = async (url: string): Promise<string> => {
	let response = await fetch(url);
	try {
		if (response.status !== 200) throw new Error(`response status was ${response.status}`);
		if (!response) throw new Error(`Couldn't read file in - ${url}`);
	} catch (ex) {
		console.error(ex);
		return '';
	}
	const code = new Uint8Array(await response.arrayBuffer());
	const id = sha256(code);
	try {
		await codeStore.setCode(id, code);
	} catch (ex) {
		console.error(ex);
		return '';
	}
	return id;
};
