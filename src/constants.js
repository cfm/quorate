const maxProxies = 10;
const proxyKeys = [];
for (let i = 1; i <= maxProxies; i++) {
  const proxyKey = { csv: `proxy${i}`, sf: `Proxy_${i}__c` };
  proxyKeys.push(proxyKey);
}

const keys = [
  { csv: 'lastName', sf: 'LastName' },
  { csv: 'firstName', sf: 'FirstName' },
  { csv: 'padding', sf: 'Id' },
  { csv: 'lastModified', sf: 'LastModifiedDate' },
  ...proxyKeys,
];

module.exports = {
  KEYS: keys.map((k) => k.csv),
  PROXY_KEYS: proxyKeys.map((k) => k.csv),

  FIELDS: keys.map((k) => k.sf),
  PROXY_FIELDS: proxyKeys.map((k) => k.sf),

  MAX_REPRESENTATION: 2,
};
