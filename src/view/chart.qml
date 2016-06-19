import QtQuick 2.0
import QtCharts 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 1.3
ApplicationWindow {
  visible: true
  title: "Architect View"
  minimumWidth: 1200
  minimumHeight: 800 + 100
  x: 400
  y: 100

  TabView {
    id: tabs
    anchors.fill: parent
    Tab {
      id: tab
      title: "Summary"
      anchors.fill: parent
      ColumnLayout {
        anchors.fill: parent
        spacing: 0
        ChartView {
          id: chart
          Layout.minimumHeight: 400
          Layout.minimumWidth: 800
          Layout.fillWidth: true
          Layout.fillHeight: true

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
          spacing: 0

          ChartView {
            id: chartDay
            Layout.minimumWidth: 600
            Layout.minimumHeight: 400
            Layout.fillWidth: true
            Layout.fillHeight: true

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
            Layout.minimumWidth: 600
            Layout.minimumHeight: 400
            Layout.fillWidth: true
            Layout.fillHeight: true

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
    Tab {
      title: "Contributers"
      TableView {
        TableViewColumn {
          role: "date"
          title: "Year, Montth"
          width: 100
        }
        TableViewColumn {
          role: "user"
          title: "Contributer"
          width: 200
        }
        model: libraryModel
      }
      ListModel {
        id: libraryModel
        ListElement {
          date: "2016, January"
          user: "pakazaka"
        }
        ListElement {
          date: "2016, March"
          user: "White Oak"
        }
        ListElement {
          date: "2016, April"
          user: "ksakepon"
        }
      }
    }

  }
}
