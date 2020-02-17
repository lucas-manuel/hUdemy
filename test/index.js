/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example

const path = require("path");

const {
  Orchestrator,
  Config,
  combine,
  singleConductor,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/hUdemy.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(tapeExecutor(require("tape")), localOnly)
});

const dna = Config.dna(dnaPath, "course_dna");
const conductorConfig = Config.gen(
  { course_dna: dna },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);

// orchestrator.registerScenario("description of example test", async (s, t) => {
//   const { alice, bob } = await s.players(
//     { alice: conductorConfig, bob: conductorConfig },
//     true
//   );

//   // Make a call to a Zome function
//   // indicating the function, and passing it an input
//   const addr = await alice.call(
//     "myInstanceName",
//     "my_zome",
//     "create_my_entry",
//     { entry: { content: "sample content" } }
//   );

//   // Wait for all network activity to settle
//   await s.consistency();

//   const result = await bob.call("myInstanceName", "my_zome", "get_my_entry", {
//     address: addr.Ok
//   });

//   // check for equality of the actual and expected results
//   t.deepEqual(result, {
//     Ok: { App: ["my_entry", '{"content":"sample content"}'] }
//   });
// });

// orchestrator.registerScenario("Scenario 1: Hello Holo", async (s, t) => {
//   const { alice, bob } = await s.players(
//     { alice: conductorConfig, bob: conductorConfig },
//     true
//   );

//   const result_hello = await alice.call("course_dna", "courses", "hi_holo", {
//     title: "hello holochain"
//   });

//   t.ok(result_hello.Ok);
//   await s.consistency();
// });

// orchestrator.registerScenario(
//   "Scenario 2: Alice Creates Course",
//   async (s, t) => {
//     const { alice, bob } = await s.players(
//       { alice: conductorConfig, bob: conductorConfig },
//       true
//     );

//     const new_course_address = await alice.call(
//       "course_dna",
//       "courses",
//       "create_course",
//       {
//         title: "first course for scenario 2",
//         timestamp: 1
//       }
//     );

//     t.ok(new_course_address.Ok);
//     await s.consistency();
//   }
// );

// orchestrator.registerScenario(
//   "Scenario 3: Alice Deletes Course",
//   async (s, t) => {
//     const { alice, bob } = await s.players(
//       { alice: conductorConfig, bob: conductorConfig },
//       true
//     );

//     const new_course_address = await alice.call(
//       "course_dna",
//       "courses",
//       "create_course",
//       {
//         title: "First course for scenario 3",
//         timestamp: 1
//       }
//     );

//     t.ok(new_course_address.Ok);
//     await s.consistency();

//     const deleted_course_address = await alice.call(
//       "course_dna",
//       "courses",
//       "delete_course",
//       {
//         course_address: new_course_address.Ok
//       }
//     );

//     t.ok(deleted_course_address.Ok === new_course_address.Ok);
//   }
// );

orchestrator.registerScenario(
  "Scenario 4: Alice Updates Course, then Gets it",
  async (s, t) => {
    const { alice, bob } = await s.players(
      { alice: conductorConfig, bob: conductorConfig },
      true
    );

    const new_course_address = await alice.call(
      "course_dna",
      "courses",
      "create_course",
      {
        title: "First course for scenario 4",
        timestamp: 1
      }
    );

    t.ok(new_course_address.Ok);
    await s.consistency();

    const updated_course_address = await alice.call(
      "course_dna",
      "courses",
      "update_course",
      {
        title:
          "Updated course title for scenario 487654345678765456787654567898765456789876545678909876",
        course_address: new_course_address.Ok,
        modules_addresses: [],
        timestamp: 1
      }
    );

    console.dir({ updated_course_address }, { depth: null });

    const courseResult = await alice.call(
      "course_dna",
      "courses",
      "get_entry",
      {
        address: updated_course_address.Ok
      }
    );
    console.dir({ courseResult }, { depth: null });

    const courseResult2 = await bob.call("course_dna", "courses", "get_entry", {
      address: updated_course_address.Ok
    });

    console.dir({ courseResult2 }, { depth: null });
  }
);

orchestrator.run();
