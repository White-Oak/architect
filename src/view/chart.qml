import QtQuick 2.0
import QtCharts 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 1.2

ApplicationWindow {
  visible: true
  title: "Architect View"
  minimumWidth: 1200
  minimumHeight: 800

  // property int margin: 5
  // width: mainLayout.implicitWidth + 2 * margin
  // height: mainLayout.implicitHeight + 2 * margin
  // minimumWidth: mainLayout.Layout.minimumWidth + 2 * margin
  // minimumHeight: mainLayout.Layout.minimumHeight + 2 * margin

  ColumnLayout {
    width: 1200
    height: 800

    ChartView {
      id: chart
      width: 1200
      height: 400
      Layout.alignment: Qt.AlignTop

      theme: ChartView.ChartThemeBrownSand
      antialiasing: true
      title: "Additions and Deletions in repo"

      BarSeries {
        id: mySeries
        axisX: BarCategoryAxis { categories: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun" ] }
        BarSet { label: "Additions"; values: [a1, a2, a3, a4, a5, a6, a7]; color: "green" }
        BarSet { label: "Deletions"; values: [d1, d2, d3, d4, d5, d6, d7]; color: "red" }
      }
    }

    RowLayout {
      id: mainLayout
      width: 1200
      height: 400
      Layout.alignment: Qt.AlignBottom

      ChartView {
        id: chartDay
        width: 600
        height: 400

        title: "Commits in repo by day"
        legend.visible: false
        antialiasing: true

        PieSeries {
          id: pieSeriesDay
          PieSlice { label: "Mon"; labelVisible:true; value: c1 }
          PieSlice { label: "Tue"; labelVisible:true; value: c2  }
          PieSlice { label: "Wed"; labelVisible:true; value: c3  }
          PieSlice { label: "Thu"; labelVisible:true; value: c4  }
          PieSlice { label: "Fri"; labelVisible:true; value: c5  }
          PieSlice { label: "Sat"; labelVisible:true; value: c6  }
          PieSlice { label: "Sun"; labelVisible:true; value: c7  }
        }
      }

      ChartView {
        id: chartTime
        width: 600
        height: 400

        title: "Commits in repo by time"
        legend.visible: false
        antialiasing: true

        PieSeries {
          id: pieSeriesTime
          PieSlice { label: "Morning"; labelVisible:true; value: cdt1 }
          PieSlice { label: "Day"; labelVisible:true; value: cdt2  }
          PieSlice { label: "Evening"; labelVisible:true; value: cdt3  }
          PieSlice { label: "Night"; labelVisible:true; value: cdt4  }
        }
      }
    }
  }
}
