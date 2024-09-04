<template>

    <h1>Audio Visualizer</h1>
    <button @click="sendTestData">Send Test Data</button>
    <button @click="micRealTime">Mic Real time</button>
    <div id="chart"></div>
</template>

<script setup lang="ts">
import { io } from "socket.io-client";
import { onMounted, ref, watch } from "vue";

import Plotly from 'plotly.js-dist';



let socket = io('127.0.0.1:8008');

socket.on("connect", () => {
    console.log("connected")
});

socket.on("testData", (inputData) => {
    // console.log("testData")
    // console.log(inputData.data)
    data.value[0].y = inputData['data']
});



socket.on("audioData", (inputData) => {
    data.value[0].y = inputData['data']
});


function sendTestData() {
    console.log("here")
    // data.value[0].y = [1, 1, 1, 1, 1]

    // socket.emit("mic")



    socket.emit("sendTestData")
}
function micRealTime() {
    console.log("here")
    // data.value[0].y = [1, 1, 1, 1, 1]

    while (true) {
        socket.emit("mic")
    }

    // setInterval(() => { socket.emit("mic") });

}



let data = ref([{
    y: [4, 5, 2, 3, 6],
    type: "bar",
    name: "asdfasfd"
}]);


var layout = {
    title: 'Standard Bar Chart',
    xaxis: {
        title: 'X Axis Title',
        tickangle: -45
    },
    yaxis: {
        title: 'Y Axis Title',
        range: [-.2, .2]
    },
    barmode: 'group',
    bargap: 0.15,
    bargroupgap: 0.1
};

onMounted(() => {
    updateChart();
})


watch(data.value, (newValue, oldValue) => {
    updateChart()
})

function updateChart() {
    Plotly.newPlot("chart", data.value, layout);
}


</script>