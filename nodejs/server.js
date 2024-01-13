const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const PROTO_PATH = "../proto/helloworld.proto";
var assert = require('assert');

const loaderOptions = {
    keepCase: true,
    longs: String,
    enums: String,
    defaults: true,
    oneofs: true,
};

let packageDef = protoLoader.loadSync(PROTO_PATH, loaderOptions);
const grpcObj = grpc.loadPackageDefinition(packageDef);

let helloworld = grpcObj.helloworld;

function doUpper(call, callback) {
    callback(null, {
	uppercased: call.request.original.toUpperCase()
    });
}

function getServer() {
    let server = new grpc.Server();
    server.addService(helloworld.Uppercase.service, {
	upper: doUpper,
    });
    return server;
}

if (require.main === module) {
  var server = getServer();
  server.bindAsync(
    '127.0.0.1:50051', grpc.ServerCredentials.createInsecure(), (err, port) => {
      assert.ifError(err);
      server.start();
  });
}

exports.getServer = getServer;
