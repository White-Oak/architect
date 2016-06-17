import QtQuick 2.0
import QtCharts 2.0

import QtQuick.Controls 1.2
ApplicationWindow {

  visible: true
  title: "Architect View"

  ChartView {
      width: 800
      height: 500
      theme: ChartView.ChartThemeBrownSand
      antialiasing: true

      BarSeries {
          id: mySeries
          axisX: BarCategoryAxis { categories: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun" ] }
          BarSet { label: "Additions"; values: [a1, a2, a3, a4, a5, a6, a7]; color: "green" }
          BarSet { label: "Deletions"; values: [d1, d2, d3, d4, d5, d6, d7]; color: "red" }
      }
  }
}
