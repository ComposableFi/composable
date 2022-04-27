import {Store} from "@subsquid/substrate-processor";

export async function get<T extends { id: string }>(
    store: Store,
    EntityConstructor: EntityConstructor<T>,
    id: string
): Promise<T | undefined> {
    return await store.get<T>(EntityConstructor, {
        where: {id},
    });
}

export async function getOrCreate<T extends { id: string }>(
    store: Store,
    EntityConstructor: EntityConstructor<T>,
    id: string
): Promise<T> {
    let entity = await store.get<T>(EntityConstructor, {
        where: { id },
    });

    if (entity == null) {
        entity = new EntityConstructor();
        entity.id = id;
    }

    return entity;
}

export type EntityConstructor<T> = {
    new (...args: any[]): T;
};
