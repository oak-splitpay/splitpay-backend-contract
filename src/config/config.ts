export const config = {
  app: {
    port: process.env.PORT || 3001,
    env: process.env.NODE_ENV || 'development',
  },
  stellar: {
    network: process.env.STELLAR_NETWORK || 'testnet',
    horizonUrl: process.env.HORIZON_URL || 'https://horizon-testnet.stellar.org',
  },
};
