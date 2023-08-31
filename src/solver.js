import solver from '@api/proxy-solver';

solver.server(process.env.VUE_APP_PROXY_SOLVER_API);

export default solver;
