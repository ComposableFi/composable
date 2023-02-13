export function getFnftKey(collectionId: string, instanceId: string) {
  return [collectionId, instanceId].join("::");
}
