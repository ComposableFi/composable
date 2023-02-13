export function getRewardKey(collectionId: string, instanceId: string) {
  return [collectionId, instanceId].join("::");
}