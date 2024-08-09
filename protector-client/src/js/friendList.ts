const tableData = [
    {
        name: 'tom1',
        id: "1",
        img_heard:'https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg'
    },
    {
        name: 'tom2',
        id: "2",
        img_heard:'https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg'
    },
    {
        name: 'Tom3',
        id: "3",
        img_heard:'https://fuss10.elemecdn.com/e/5d/4a731a90594a4af544c0c25941171jpeg.jpeg'
    },

]

// 点击列表事件
function click_list(row: any, column: any, event: Event) {
    console.log(row.id)
    console.log(column)
    console.log(event)
}

export {
    click_list,
    tableData
}